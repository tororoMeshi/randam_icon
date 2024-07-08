# random_icon
This project is a web application that generates random icons using geometric shapes. It provides an API endpoint to generate icons and return them as PNG images.

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Logging](#logging)
- [Kubernetes Deployment](#kubernetes-deployment)
- [Sample Icon](#sample-icon)
- [License](#license)

## Introduction
The Random Icon Generator API generates random icons composed of various geometric shapes, each with random sizes, colors, and orientations. The API is built with Rust and Actix Web, and it logs each request with a unique UUID for traceability.

## Features
- Generates random icons using geometric shapes
- Supports multiple shape types (circle, semi-circle, square, pentagon, hexagon)
- Randomly selects colors from a predefined palette
- Logs requests with unique UUIDs for traceability

## Installation
To build and run the API locally, follow these steps:
1. Ensure you have Rust installed.
2. Clone the repository:
```sh
git clone git@github.com:tororoMeshi/random_icon.git
cd random_icon/random_icon
```
3. Build the project:
```sh
cargo build --release
```
4. Run the application:
```sh
cargo run
```

## Usage
The API provides a single endpoint to generate an icon:

- GET `/generate-icon`
### Example Request
```sh
curl http://localhost:8080/generate-icon --output icon.png
```
This will save the generated icon to a file named `icon.png`.

## Configuration
You can configure the logging level using the `RUST_LOG` environment variable:
```sh
export RUST_LOG=info
```

Possible values are `error`, `warn`, `info`, `debug`, and `trace`.

## Logging
The application uses `flexi_logger` for logging. Logs include information about each request and response, as well as any errors that occur during icon generation.

## Kubernetes Deployment
To deploy the application on Kubernetes, follow these steps:
1. Build and push the Docker image to your container registry:
```sh
cd random_icon/random_icon
docker build -t yourusername/random-icon-generator .
docker push yourusername/random-icon-generator
```
2. Apply the Kubernetes manifests:
```sh
kubectl apply -f yaml/deployment.yaml
kubectl apply -f yaml/service.yaml
```
### Example Kubernetes Manifest
deployment.yaml
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: random-icon-generator
spec:
  replicas: 2
  selector:
    matchLabels:
      app: random-icon-generator
  template:
    metadata:
      labels:
        app: random-icon-generator
    spec:
      containers:
      - name: random-icon-generator
        image: yourusername/random-icon-generator
        ports:
        - containerPort: 8080
```
service.yaml
```yaml
apiVersion: v1
kind: Service
metadata:
  name: random-icon-generator
spec:
  type: NodePort
  selector:
    app: random-icon-generator
  ports:
    - protocol: TCP
      port: 8080
      targetPort: 8080
      nodePort: 30036
```
3. Access the API using the node's IP address and the NodePort defined in the service manifest.

## Sample Icon
### Sample 1
![generate-icon-0.png](https://github.com/tororoMeshi/random_icon/blob/main/img/generate-icon-0.png)

### Sample 2
![generate-icon-1.png](https://github.com/tororoMeshi/random_icon/blob/main/img/generate-icon-1.png)

### Sample 3
![generate-icon-2.png](https://github.com/tororoMeshi/random_icon/blob/main/img/generate-icon-2.png)

## License
This project is licensed under the MIT License. See the [LICENSE](https://github.com/tororoMeshi/random_icon/blob/main/LICENSE) file for details.

