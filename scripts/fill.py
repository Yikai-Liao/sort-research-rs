"""Refresh derived tables; reviewed drafts are edited separately.

The old implementation rewrote drafts with incorrect cache and pass-count claims.
Use results/TABLES.md for the complete regenerated historical tables.
"""
from refresh import refresh
if __name__ == "__main__":
    refresh()
