# How to use the s3-db tool?

The s3-db tool is a utility to create cryptographic material databases and store them on an S3 bucket. Essentially, it enables users to utilize their self-signed certificates/keys for testing purposes.
Before using the tool, the key materials must be created in advance.

To create a private key, use the following command:

`openssl genrsa -out my-key.pem 2048`

To create your self-signed certificate, you can use this command:

`openssl req -x509 -new -nodes -key my-key.pem -sha256 -days 365 -out "my-cert.pem" -subj "/CN=www.acm4ne.com"`

To create a database, we need to call the s3-db utility with its create command:

```
s3-db create --kms-key-id <your-kms-key-id> \
             --kms-region <your-kms-region> \
             --certificate-path my-cert.pem \
             --private-key-path my-key.pem \
             --output-path <your-db-path>
```

When S3 is chosen as the source and a valid bucket name is provided in the configuration file (/etc/nitro_enclaves/acm.yaml), the ACMNE agent searches for a certificate in that bucket named acmne_test.db.

For example, when `aws-ec2-enclave-certificate-test` is provided as the S3 bucket name, the resulting URI becomes:

```
s3://aws-ec2-enclave-certificate-test/acmne_test.db
```

Using this URI, we can push our database to the S3 bucket:

```
s3-db push --s3-uri s3://aws-ec2-enclave-certificate-test/acmne_test.db --s3-region <your-s3-region> --input-path <your-db-path>
```

Now, you are ready to use your cryptographic material database for testing. Finally, the following policy must be attached to the IAM role associated with the instance:

```
{
    "Effect": "Allow",
    "Action": [
        "s3:GetObject",
        "s3:ListAccessGrants"
    ],
    "Resource": [
        "arn:aws:s3:::aws-ec2-enclave-certificate-test",
        "arn:aws:s3:::aws-ec2-enclave-certificate-test/*"
    ]
}
```
