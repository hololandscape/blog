

.PHONY: run
run:
	@echo "Running server..."
	@cargo run --bin server

.PHONY: test
test:
	@echo "Running tests..."
	@grpcurl -plaintext -import-path ./backend/proto -proto demo.proto -d '{"name": "Tonic"}' '[::1]:50051' demo.Demo/SayHello