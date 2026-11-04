// Shared functionality coverage. Comparator-only tests and owning records are
// outside these fixed-width radix adapters; rejection is tested explicitly.
macro_rules! radix_functionality {
    ($sort:ty) => {
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, basic);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, fixed_seed);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, fixed_seed_rand_vec_prefix);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, int_edge);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_i32_random);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_i32_random_z1);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_i32_random_d2);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_i32_random_d20);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_i32_random_s95);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_i32_ascending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_i32_descending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_i32_saw_mixed);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_i32_random_d4);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_i32_random_d8);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_i32_random_d311);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_i32_random_d1024);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_i32_random_z1_03);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_i32_random_z2);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_i32_random_s50);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_i32_narrow);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_i32_all_equal);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_i32_saw_mixed_range);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_i32_pipe_organ);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_u64_random);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_u64_random_z1);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u64_random_d2);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u64_random_d20);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_u64_random_s95);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u64_ascending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u64_descending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u64_saw_mixed);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u128_random);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_u128_random_z1);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u128_random_d2);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u128_random_d20);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u128_random_s95);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u128_ascending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u128_descending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_u128_saw_mixed);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_cell_i32_random);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_cell_i32_random_z1);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_cell_i32_random_d2);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_cell_i32_random_d20);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_cell_i32_random_s95);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_cell_i32_ascending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_cell_i32_descending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_cell_i32_saw_mixed);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_f128_random);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_f128_random_z1);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_f128_random_d2);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_f128_random_d20);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_f128_random_s95);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_f128_ascending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_f128_descending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_f128_saw_mixed);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_1k_random);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, correct_1k_random_z1);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_1k_random_d2);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_1k_random_d20);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_1k_random_s95);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_1k_ascending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_1k_descending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, correct_1k_saw_mixed);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, stability_i32_random);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_yes, stability_i32_random_z1);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, stability_i32_random_d2);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, stability_i32_random_d20);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, stability_i32_random_s95);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, stability_i32_ascending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, stability_i32_descending);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, stability_i32_saw_mixed);
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, stability_cell_i32_random);
        sort_test_tools::instantiate_sort_test_inner!(
            $sort,
            miri_yes,
            stability_cell_i32_random_z1
        );
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, stability_cell_i32_random_d2);
        sort_test_tools::instantiate_sort_test_inner!(
            $sort,
            miri_no,
            stability_cell_i32_random_d20
        );
        sort_test_tools::instantiate_sort_test_inner!(
            $sort,
            miri_no,
            stability_cell_i32_random_s95
        );
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, stability_cell_i32_ascending);
        sort_test_tools::instantiate_sort_test_inner!(
            $sort,
            miri_no,
            stability_cell_i32_descending
        );
        sort_test_tools::instantiate_sort_test_inner!($sort, miri_no, stability_cell_i32_saw_mixed);
    };
}
