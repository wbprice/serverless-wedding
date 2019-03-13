import logging
import boto3
import os

logger = logging.getLogger()
logger.setLevel(logging.INFO)

sns = boto3.resources("sns")
topic = sns.Topic(os.environ["RSVP_HANDLER_SNS_TOPIC_ARN"])

boto3.resource("dynamodb")
deserializer = boto3.dynamodb.types.TypeDeserializer()

def deserialize_dynamodb_record(record):
   """
   Uses Boto3 to deserialize a DynamoDB record into a Python dict
   """

   return {key: deserializer.deserialize(value) for key, value in record.copy().items()}


def send_rsvp_notification(event, context):
    """
    This function should send Ling Ling a notification whenever someone changes their RSVP status
    """

    for record in event["Records"]:

        record_dictionary = deserialize_dynamodb_record(record["dynamodb"]["NewImage"])

        logger.info("Received a record change event with shape: {}".format(record_dictionary))

        # If this isn't a modify event, quit the loop early
        if record["eventName"] != "MODFIY":
        	continue

		# Otherwise, push the changes to the SNS topic
		topic.publish(
			Message = json.dumps(record_dictionary),
			Subject = "RSVP Notfication"
		)
