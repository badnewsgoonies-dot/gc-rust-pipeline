//! Trial battery — one module per stored intent body.
pub mod trial1;
pub mod trial2;
pub mod trial3;
pub mod trial4;
pub mod trial5;
// pub mod trial6;   // FAILS — bullet echo, parse error at line 5 `impl GenerationLog:`
pub mod trial6b;
pub mod trial7;
pub mod trial8;
pub mod trial9;
pub mod fmt_bul;  // Sonnet 4.6 + bullet format spec
pub mod fmt_num;  // Sonnet 4.6 + numbered (1)(2)(3) format spec
pub mod trial6_sonnet;  // Sonnet 4.6 + literal T6 prompt that broke Haiku
pub mod fmt_bul_haiku;  // Haiku 4.5 + bullet format BoundedQueue (load-bearing 2x2 cell)
// pub mod opus_t6_01;  // FAILS — Debug bound
// pub mod gpt54_t6_01;  // FAILS — same Debug bound
pub mod opus_t6_fixed;
pub mod haiku_t6_fixed;
pub mod ring_buffer_v1;
pub mod ring_buffer_codex;

pub mod par_lru;
pub mod par_heap;
pub mod par_bitset;
pub mod par_chunks;
pub mod b2_stack;
pub mod b2_queue;
pub mod b2_interval;
pub mod b2_rle;
pub mod b2_unionfind;
pub mod b2_graph;pub mod b3_btree_set;
pub mod b3_deque_buf;
pub mod b3_bloom_filter;
pub mod b3_prefix_sum;
pub mod b3_bits_packed;
pub mod b3_lcg_rng;
pub mod b3_bit_reverse;
pub mod b3_run_avg;
pub mod b3_sliding_max;
pub mod b3_counter_map;
pub mod b3_base64_codec;
pub mod b3_parse_csv;
pub mod b4_segtree;
pub mod b4_fenwick;
pub mod b4_sieve;
pub mod b4_levenshtein;
pub mod b4_vec3f;
pub mod b4_complex;
pub mod b4_fraction;
pub mod b4_polynomial;
pub mod b4_kmp;
pub mod b4_reservoir;
pub mod b4_histogram;
pub mod b4_topk;
pub mod b4_bitvec;
pub mod b4_color;
pub mod b4_sortedvec;
pub mod b4_anagram;
pub mod b4_rle_str;
pub mod b4_counting_sort;
pub mod b4_hashmap_lp;
pub mod b4_charfreq;
