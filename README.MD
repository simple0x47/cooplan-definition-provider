# Definition Provider

## Description

Microservice designed to provide definitions to API Gateways and other microservices.

## API

This microservice's API is exposed through a RabbitMQ instance.

There is an input queue and an output queue:

* Input: 
  * `definition`
    * `get(version)`: obtains the definition for the specified version.
* Output:
  * `latest_definition`: the latest definition is retrievable always from this queue that is a stream.