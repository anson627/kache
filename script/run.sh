echo "Running go benchmark..."
pushd go
/usr/bin/time -l go test -bench . -benchtime=1m
popd

echo "Running rust benchmark..."
pushd rust
/usr/bin/time -l cargo bench -- --measurement-time 60
popd