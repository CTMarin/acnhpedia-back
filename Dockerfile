# Build acnhpedia backend
FROM rust:buster AS base
ADD . ./acnhpedia-back
WORKDIR ./acnhpedia-back
EXPOSE 5000
CMD [ "cargo", "run", "--release" ]

