echo "Running go benchmark..."
pushd go
go test -bench .
popd

echo "Running rust benchmark..."
pushd rust
cargo bench
popd