# Project TODO

## COMPLETED (Finalized)

### Gemini Curator
- [x] Establish gemini-curator extension structure.
- [x] Integrate team-cli-source vault and Conductor templates.
- [x] Implement evaluator_pro.py with D1-D8 scoring logic.
- [x] Implement disk_cleanup.py for automated maintenance.
- [x] Create /curator command for CLI integration.
- [x] Setup herness (eval-harness) with executable scripts.
- [x] Verify Architect Mandates (Rust Style & Efficiency) enforcement.

### Formula Engine V2
- [x] Phase 8: Access Chaining & Indexing (Property/Index access)
- [x] Phase 9: Lambda & Higher-Order Functions (`map`, `filter`, `reduce` with closures)
- [x] Phase 10: User-Defined Functions (With recursive short-circuiting / lazy `if` fix)
- [x] Phase 13: Plugin SDK Foundation (`Plugin` trait and `PluginManager` integration)

## NEXT STEPS (Future Maintenance)

### Gemini Curator
- [ ] Schedule auditor.sh via Cron for periodic health checks.
- [ ] Expand pattern_detector.py for more complex semantic violations.

### Formula Engine V2
- [ ] Phase 11: Advanced Data Types (jiff native DateTime/Duration, Set, Range)
- [ ] Phase 12: Serialization & Caching (Serde derive, FormulaCache)
- [ ] Phase 14: Performance & Optimization (Constant folding, benchmarks)
- [ ] Phase 15: Error Recovery + Security Limits (Recovery parsing, execution limits)
