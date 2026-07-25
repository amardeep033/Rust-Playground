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

## Next In Line
| question | what it tests |
|---|---|
| 9_datafusion_parquet | DataFusion session context, Arrow schemas, SQL/DataFrame APIs, partitioned Parquet queries, predicate/projection pushdown, schema mismatch handling, query metrics |
| 10_otel_observability | OpenTelemetry traces, metrics, spans, latency/error visibility |
| 11_external_redis_integration | Redis crate/client usage, connection config, commands, TTL, error handling, retries |
| 12_external_kafka_integration | Kafka crate/client usage, producer/consumer flow, topics, offsets, consumer groups, retries |
| 13_external_s3_integration | AWS S3 + MinIO client usage, endpoint/region/credentials, path-style/virtual-host addressing, upload/download, structured errors |
| 14_external_clickhouse_integration | ClickHouse client usage, batch insert, schema validation, query execution, ingestion errors |
| 15_external_rabbitmq_integration | AMQP client usage, exchanges, queues, bindings, publish/consume, ack/nack, reconnect handling |
| 16_kv_store_ttl | in-memory key-value store, commands, TTL expiry, cleanup strategy, ownership-aware API |
| 17_append_only_wal | append-only log, record encoding, fsync policy, replay, recovery from partial writes |
| 18_rate_limiter | token bucket/sliding window, concurrency, API design |
| 19_lru_cache | HashMap + custom doubly linked structure, recency updates, eviction, ownership-aware cache API |
| 20_worker_pool_scheduler | bounded queues, round-robin/least-loaded assignment, task priorities, backpressure, worker health, graceful shutdown, task redistribution |
| 21_retry_timeout_backoff | async reliability patterns, timeout handling, retry policy |
| 22_idempotency_layer | duplicate request handling, consistency, request state |
| 23_circuit_breaker | state machine, failure tracking, half-open recovery |
| 24_config_driven_executor | trait-based execution, orchestration, resume-aligned platform design |
| 25_mini_kafka_log | topic partitions, append-only segment log, offsets, producer/consumer model, recovery |
| 26_dag_job_scheduler | graph traversal, dependency execution, orchestration |
| 27_grpc_webhook_service | gRPC service, webhook delivery, signing, retry semantics |
| 28_in_memory_rabbitmq | in-memory broker, exchanges, queues, bindings, competing consumers, ack/nack, requeue, dead-lettering, delivery attempts |
| 29_consistent_hashing | hash ring, sharding, node add/remove behavior |
| 30_profiling_benchmarking | Criterion, flamegraph, throughput/latency reasoning |
| 31_tui_search_tool | Ratatui, fuzzy search, filesystem traversal, interactive UI |
| 32_ownership_aware_apis | borrowing vs ownership choices, lifetimes, cloning tradeoffs |
| 33_tlv_ttl_protocol | binary parsing, protocol design, TTL semantics |
| 34_trait_based_errors | JSON errors, domain errors, trait-based/OOP-style design |
