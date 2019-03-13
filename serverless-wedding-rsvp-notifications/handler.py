import logging
import boto3

logger = logging.getLogger()
logger.setLevel(logging.INFO)

boto3.resource('dynamodb')
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
