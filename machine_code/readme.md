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
| 9_trait_based_errors | JSON errors, domain errors, trait-based/OOP-style design |

## Next In Line
| question | what it tests |
|---|---|
| n_datafusion_parquet | DataFusion session context, Arrow schemas, SQL/DataFrame APIs, partitioned Parquet queries, predicate/projection pushdown, schema mismatch handling, query metrics |
| n_otel_observability | OpenTelemetry traces, metrics, spans, latency/error visibility |
| n_external_redis_integration | Redis crate/client usage, connection config, commands, TTL, error handling, retries |
| n_external_kafka_integration | Kafka crate/client usage, producer/consumer flow, topics, offsets, consumer groups, retries |
| n_external_s3_integration | AWS S3 + MinIO client usage, endpoint/region/credentials, path-style/virtual-host addressing, upload/download, structured errors |
| n_external_clickhouse_integration | ClickHouse client usage, batch insert, schema validation, query execution, ingestion errors |
| n_external_rabbitmq_integration | AMQP client usage, exchanges, queues, bindings, publish/consume, ack/nack, reconnect handling |
| n_kv_store_ttl | in-memory key-value store, commands, TTL expiry, cleanup strategy, ownership-aware API |
| n_append_only_wal | append-only log, record encoding, fsync policy, replay, recovery from partial writes |
| n_rate_limiter | token bucket/sliding window, concurrency, API design |
| n_lru_cache | HashMap + custom doubly linked structure, recency updates, eviction, ownership-aware cache API |
| n_worker_pool_scheduler | bounded queues, round-robin/least-loaded assignment, task priorities, backpressure, worker health, graceful shutdown, task redistribution |
| n_retry_timeout_backoff | async reliability patterns, timeout handling, retry policy |
| n_idempotency_layer | duplicate request handling, consistency, request state |
| n_circuit_breaker | state machine, failure tracking, half-open recovery |
| n_config_driven_executor | trait-based execution, orchestration, resume-aligned platform design |
| n_mini_kafka_log | topic partitions, append-only segment log, offsets, producer/consumer model, recovery |
| n_dag_job_scheduler | graph traversal, dependency execution, orchestration |
| n_grpc_webhook_service | gRPC service, webhook delivery, signing, retry semantics |
| n_in_memory_rabbitmq | in-memory broker, exchanges, queues, bindings, competing consumers, ack/nack, requeue, dead-lettering, delivery attempts |
| n_consistent_hashing | hash ring, sharding, node add/remove behavior |
| n_profiling_benchmarking | Criterion, flamegraph, throughput/latency reasoning |
| n_tui_search_tool | Ratatui, fuzzy search, filesystem traversal, interactive UI |
| n_ownership_aware_apis | borrowing vs ownership choices, lifetimes, cloning tradeoffs |
| n_tlv_ttl_protocol | binary parsing, protocol design, TTL semantics |
