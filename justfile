dockerbuild:
    docker build -t daicanglong/genshin-dictionary-backend:latest . --platform linux/x86_64

dockerpush:
    docker push daicanglong/genshin-dictionary-backend:latest

dockerbuildandpush: dockerbuild dockerpush

test:
    cargo test --workspace

fix:
    cargo clippy --fix --allow-dirty