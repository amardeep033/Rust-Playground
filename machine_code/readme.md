## Rules

- Time-box: 30 min per question
- No AI assistance -- docs/search only
- Prioritize clear naming and proper error handling
- Plan structure and approach before coding
- Consider time complexity
- Read and understand references, don't copy
- Comment liberally to explain intent
- Think through edge cases / follow-up questions

## Questions (easy -> hard)

| question | what it tests |
|---|---|
| 0_log_ip_top_k | parsing, `?` error propagation, top-k heap |
| 1_log_error_count_by_service | basic iteration + HashMap counting |
| 2_csv_group_by_agg | file/CSV parsing + group-by-count |
| 3_merge_sorted_log | merge-sorted-streams algorithm |
| 4_static_vs_dynamic_dispatch | static vs dynamic dispatch, trait objects |
| 5_json_parse_error_types | JSON parsing + custom error types |
| 6_concurrent_service_counter | shared mutable state across async tasks |
| 7_async_pipeline_channels | multi-stage async pipeline via channels |
| 8_book_scraper_server | actix-web service + live scraping (reqwest) |