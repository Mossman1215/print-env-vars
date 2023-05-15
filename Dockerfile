FROM quay.io/fedora/fedora:38

COPY target/release/print-env-web /usr/bin/print-env-web

CMD print-env-web
