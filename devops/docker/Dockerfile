FROM rabbitmq:3-management

EXPOSE 1883
EXPOSE 15672

COPY ./data/definitions.json /etc/rabbitmq/definitions.json
COPY ./data/enabled_plugins /etc/rabbitmq/enabled_plugins
