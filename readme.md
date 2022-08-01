# serde_json on armv7

Here is an example of code and payload working on amd64, arm64 but not on armv7, throwing the following error

```
Error("data did not match any variant of untagged enum Entry", line: 305, column: 1)
```

## How to reproduce

You can run it on a Raspberry PI with an armv7 processor and you'll have the error (I tried it), but you can emulate this with docker

```bash
# for amd64
docker buildx build -t serde-json-armv7 --platform linux/amd64 .
# for arm64
docker buildx build -t serde-json-armv7 --platform linux/arm64 .
# for armv7
docker buildx build -t serde-json-armv7 --platform linux/arm/v7 .
```