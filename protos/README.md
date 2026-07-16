# Protobuf definitions for GOnnect chat bridge

## Git pre-commit hook

Add this as executable file `.git/hooks/pre-commit`:

```sh
#!/usr/bin/sh

protoc --fatal_warnings --cpp_out=.out *.proto
```

to validate the proto files before committing.