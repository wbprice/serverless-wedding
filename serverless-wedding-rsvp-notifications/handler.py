import logging
import boto3
import os
import json

logger = logging.getLogger()
logger.setLevel(logging.INFO)

boto3.resource("dynamodb")
deserializer = boto3.dynamodb.types.TypeDeserializer()
sns = boto3.client("sns")

SNS_TOPIC_ARN = os.environ["RSVP_HANDLER_SNS_TOPIC_ARN"]

def deserialize_dynamodb_record(record):
   """
   Uses Boto3 to deserialize a DynamoDB record into a Python dict
   """

   return {key: deserializer.deserialize(value) for key, value in record.copy().items()}


def create_message_text(record):
    person = record["name"]
    rsvp = "YES" if record["attending"] else "NO"

    return f"{person} RSVPd {rsvp} to the wedding invite!"


def send_rsvp_notification(event, context):
    """
    This function should send Ling Ling a notification whenever someone changes their RSVP status
    """

    for record in event["Records"]:

        record_dictionary = deserialize_dynamodb_record(record["dynamodb"]["NewImage"])

        logger.info("Received a record change event with shape: {}".format(record_dictionary))

        # If this isn't a modify event, quit the loop early
        if record["eventName"] != "MODIFY":
            continue

        # Otherwise, push the changes to the SNS topic
        try:
            response = sns.publish(
                TopicArn = SNS_TOPIC_ARN,
                Message = create_message_text(record_dictionary),
                Subject = "RSVP Notfication"
            )

            logger.info("Published a message to the SNS topic with ARN: {}".format(SNS_TOPIC_ARN))
            logger.info("The response was: {}".format(response))
        except Exception as e:
            logger.error("There was an error publishing to the topic: {}".format(e))

