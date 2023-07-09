## Introduction

This is a simple container image to send emails via SMTP.

It is very small, the compressed size is only 3.92MB in x86_64 architecture, thus is very suitable for use in CI.

You can use it like this:

```bash
docker run --name mailer --rm \
    -e MAILER_FROM_ADDRESS="user@example.com" \
    -e MAILER_FROM_NAME="Your Name" \
    -e MAILER_RECIPIENTS="foo@gmail.com,bar@outlook.com" \
    -e MAILER_SUBJECT="Hello!" \
    -e MAILER_BODY="This is\nthe body." \
    -e MAILER_BODY_IS_HTML="false" \
    -e MAILER_ATTACHMENT_PATH="/path/to/file" \
    -e MAILER_ATTACHMENT_TYPE="text/plain" \
    -e MAILER_USER_NAME="user@example.com" \
    -e MAILER_PASSWORD="password" \
    -e MAILER_HOST="smtp.example.com" \
    -e MAILER_PORT="465" \
    -e MAILER_USE_STARTTLS="false" \
    ghcr.io/sainnhe/mailer:latest
```

The executable is placed in `/usr/local/bin/mailer`, you can manually run it in a container.

This image is available in the following registries:

- [`ghcr.io`](https://github.com/sainnhe/container-mailer/pkgs/container/mailer)
- [`quay.io`](https://quay.io/repository/sainnhe/mailer)

## Environment Variables

- `MAILER_FROM_ADDRESS`: Send email from this address.
- `MAILER_FROM_NAME`: The name of the sender.
- `MAILER_RECIPIENTS`: Comma separated list of recipients to send the mail to.
- `MAILER_SUBJECT`: The subject.
- `MAILER_BODY`: The body. Use `\n` to break new line.
- `MAILER_BODY_IS_HTML` (optional): If set to `"true"`, `MAILER_BODY` is rendered in HTML. If set to `"false"`, `MAILER_BODY` is rendered in plain text. Default to `"false"`.
- `MAILER_ATTACHMENT_PATH` (optional): Attachment file path. Default to `""`.
- `MAILER_ATTACHMENT_TYPE` (optional): Attachment file type. One of media types in [https://www.iana.org/assignments/media-types/media-types.xhtml](https://www.iana.org/assignments/media-types/media-types.xhtml). Default to `"text/plain"`.
- `MAILER_USER_NAME`: SMTP user name.
- `MAILER_PASSWORD`: SMTP password.
- `MAILER_HOST`: SMTP host.
- `MAILER_PORT`: SMTP port.
- `MAILER_USE_STARTTLS`: If set to `"false"`, use TLS. If set to `"true"`, use STARTTLS.

## Example

Here is an example of using this image in [Woodpecker CI](https://woodpecker-ci.org):

```yaml
branches: master

pipeline:
  test:
    image: <your-image>
    commands:
      - <your-test-commands>
  notify:
    image: ghcr.io/sainnhe/mailer:latest
    commands:
      - mailer
    secrets:
      [
        MAILER_FROM_ADDRESS,
        MAILER_FROM_NAME,
        MAILER_RECIPIENTS,
        MAILER_USER_NAME,
        MAILER_PASSWORD,
        MAILER_HOST,
        MAILER_PORT,
        MAILER_USE_STARTTLS,
      ]
    environment:
      - MAILER_SUBJECT=Run Failed
      - MAILER_BODY=${CI_BUILD_LINK}
    when:
      status:
        - failure
```

## License

[GPL3](./LICENSE) © sainnhe
