# Single Responsibility Principle (SRP) Restructuring Plan

This document outlines the plan for restructuring the Venice AI API SDK to better follow the Single Responsibility Principle (SRP).

## Motivation

The current structure has several issues:
- Duplicate types like `traits::chat::ChatCompletionRequest` and `chat::completions::ChatCompletionRequest`
- Client handles too many responsibilities (HTTP requests, rate limiting, retries, etc.)
- Unclear separation between public API and internal implementation

## Restructuring Plan

### Phase 1: Core Infrastructure (✅ Completed)

- [x] Create HTTP client layer (`src/http/client.rs`)
- [x] Implement middleware pattern (`src/middleware/mod.rs`, `src/middleware/rate_limiter.rs`, `src/middleware/retry.rs`)
- [x] Set up basic directory structure

### Phase 2: Model Consolidation (✅ Completed)

- [x] Create models directory
- [x] Create unified `ChatCompletionRequest` type in `models/chat.rs`
- [x] Implement conversions between different `ChatCompletionRequest` types
- [x] Add tests for conversions

### Phase 3: API Implementation (✅ Completed)

- [x] Create API services for chat API (`src/api/chat.rs`)
- [x] Create API services for models API (`src/api/models.rs`)
- [x] Create API services for image API (`src/api/image.rs`)
- [x] Create API services for API keys API (`src/api/api_keys.rs`)
- [x] Update client to use new API services
- [x] Implement trait delegation in client

### Phase 4: Utility Services (✅ Completed)

- [x] Move webhook verification to a service
- [x] Create services module structure
- [x] Update references to use new services

### Phase 5: Testing and Documentation (🔄 In Progress)

- [x] Fix broken tests
- [x] Add tests for new components
- [ ] Update documentation

## Implementation Details

### API Services

Each API service follows this pattern:
- Implements the corresponding trait from `src/traits/`
- Takes a `SharedHttpClient` in its constructor
- Handles the specific API endpoints
- Converts between API types and response types

### Client Changes

The client now:
- Delegates API calls to the appropriate API service
- Maintains backward compatibility through the trait implementations
- Provides a cleaner separation of concerns

## Benefits

This restructuring:
1. Reduces code duplication
2. Makes the codebase more maintainable
3. Improves testability
4. Provides clearer separation of concerns
5. Makes it easier to add new features

## Next Steps

1. Update documentation
2. Add more examples
3. Consider implementing more utility services
4. Improve error handling