// Added competitor registrations. Upstream functions, inputs and timed loops stay intact.
template<class T>
void tessera_and_tie(InlinePermuteItem<T>* items,size_t n,Tie& tie,bool& has_ties) {
    call_tessera(items,n);
    using U=typename RadixSortTraits<T>::UnsignedKey;
    detail::detect_ties_in_range<U>(reinterpret_cast<const detail::RadixPermuteItem<U>*>(items),0,n,tie,has_ties);
}

template<class T, Distribution Dist>
void BM_Tessera(benchmark::State& state) {
    const size_t n=static_cast<size_t>(state.range(0));
    const auto values=generate_data<T>(n,Dist);
    const auto initial_items=make_initial_permutation(values);
    InlinePermutation<T> items(n);
    Tie tie(n,1);
    for(auto _:state) {
        std::memcpy(items.data(),initial_items.data(),n*sizeof(InlinePermuteItem<T>));
        std::memset(tie.data(),1,n);
        tie[0]=0;
        bool range_has_ties=false;
        tessera_and_tie(items.data(),n,tie,range_has_ties);
        benchmark::DoNotOptimize(items.data());
        benchmark::DoNotOptimize(tie.data());
        benchmark::DoNotOptimize(range_has_ties);
    }
    state.SetItemsProcessed(state.iterations()*static_cast<int64_t>(n));
    state.SetBytesProcessed(state.iterations()*static_cast<int64_t>(n*sizeof(T)));
}

template<class T>
void BM_TesseraStable(benchmark::State& state) {
    const size_t n=static_cast<size_t>(state.range(0));
    const auto values=generate_data<T>(n,Distribution::LOW_CARDINALITY);
    const auto initial_items=make_initial_permutation(values);
    InlinePermutation<T> items(n);
    SmallPermutation small_perm(n);
    Tie tie(n,1);
    for(auto _:state) {
        std::memcpy(items.data(),initial_items.data(),n*sizeof(InlinePermuteItem<T>));
        std::memset(tie.data(),1,n);
        tie[0]=0;
        bool range_has_ties=false;
        tessera_and_tie(items.data(),n,tie,range_has_ties);
        restore_inline_permutation(items,small_perm);
        benchmark::DoNotOptimize(small_perm.data());
        benchmark::DoNotOptimize(tie.data());
    }
    state.SetItemsProcessed(state.iterations()*static_cast<int64_t>(n));
    state.SetBytesProcessed(state.iterations()*static_cast<int64_t>(n*sizeof(T)));
}

#define REGISTER_TESSERA_DIST(TYPE, TYPE_NAME, DIST, DIST_NAME) \
 BENCHMARK_TEMPLATE(BM_Tessera,TYPE,Distribution::DIST)->Name("BM_Tessera_" TYPE_NAME "_" DIST_NAME)->Arg(256)->Arg(1024)->Arg(4096)->Arg(65536)->Arg(1000000); \
 BENCHMARK_TEMPLATE(BM_RadixSort,TYPE,Distribution::DIST)->Name("BM_RadixSortAA_" TYPE_NAME "_" DIST_NAME)->Arg(256)->Arg(1024)->Arg(4096)->Arg(65536)->Arg(1000000)
#define REGISTER_TESSERA_TYPE(TYPE, NAME) \
 REGISTER_TESSERA_DIST(TYPE,NAME,UNIFORM_RANDOM,"UniformRandom"); \
 REGISTER_TESSERA_DIST(TYPE,NAME,ALREADY_SORTED,"AlreadySorted"); \
 REGISTER_TESSERA_DIST(TYPE,NAME,REVERSE_SORTED,"ReverseSorted"); \
 REGISTER_TESSERA_DIST(TYPE,NAME,LOW_CARDINALITY,"LowCardinality"); \
 REGISTER_TESSERA_DIST(TYPE,NAME,NON_NEGATIVE_LOW_CARD,"NonNegLowCard"); \
 REGISTER_TESSERA_DIST(TYPE,NAME,NEARLY_SORTED,"NearlySorted"); \
 REGISTER_TESSERA_DIST(TYPE,NAME,SORTED_PREFIX_THEN_RANDOM,"SortedPrefix90"); \
 REGISTER_TESSERA_DIST(TYPE,NAME,ZIPFIAN,"Zipfian"); \
 BENCHMARK_TEMPLATE(BM_TesseraStable,TYPE)->Name("BM_TesseraStable_" NAME "_LowCard")->Arg(256)->Arg(1024)->Arg(4096)->Arg(65536)->Arg(1000000)
REGISTER_TESSERA_TYPE(int32_t,"Int32");
REGISTER_TESSERA_TYPE(int64_t,"Int64");
REGISTER_TESSERA_TYPE(float,"Float");
REGISTER_TESSERA_TYPE(double,"Double");

// Independent correctness check uses the upstream comparator and original values.
// It runs before the benchmark main, outside every timing loop.
template<class T> size_t validate_type() {
    size_t checked=0;
    for(int d=0;d<8;++d)for(size_t n:{256,1024,4096,65536,1000000}) {
        const auto values=generate_data<T>(n,static_cast<Distribution>(d));
        auto original=make_initial_permutation(values);
        auto expected=original;
        std::stable_sort(expected.begin(),expected.end(),[](const auto&a,const auto&b){return SorterComparator<T>::compare(a.inline_value,b.inline_value)<0;});
        for(int algorithm=0;algorithm<3;++algorithm) {
            auto items=original;
            InlinePermutation<T> scratch(n);
            Tie tie(n,1);tie[0]=0;
            bool has_ties=false;
            if(algorithm==0)radix_sort_inlined_and_tie<T>(false,items.data(),scratch.data(),0,n,true,tie,has_ties);
            if(algorithm==1)tessera_and_tie(items.data(),n,tie,has_ties);
            if(algorithm==2){InlinePermutation<T> cold;radix_sort_inlined_and_tie<T>(false,items.data(),cold,0,n,true,tie,has_ties);}
            for(size_t i=0;i<n;++i) {
                if(items[i].index_in_chunk!=expected[i].index_in_chunk)throw std::runtime_error("permutation/stability mismatch");
                const bool equal=i>0 && SorterComparator<T>::compare(expected[i-1].inline_value,expected[i].inline_value)==0;
                if(bool(tie[i])!=equal)throw std::runtime_error("tie mismatch");
            }
            ++checked;
        }
    }
    return checked;
}
struct ValidateBeforeMain {
    ValidateBeforeMain() {
        const size_t checked=validate_type<int32_t>()+validate_type<int64_t>()+validate_type<float>()+validate_type<double>();
        std::cerr<<"Validated "<<checked<<" upstream type/distribution/size/algorithm cases against stable_sort\n";
    }
};
static ValidateBeforeMain validation;
