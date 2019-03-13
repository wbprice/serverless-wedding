import logging

logger = logging.getLogger()
logger.setLevel(logging.INFO)


def send_rsvp_notification(event, context):
    """
    This function should send Ling Ling a notification whenever someone changes their RSVP status
    """

    for record in event["Records"]:

        logger.info("Received a record change event with shape: {}".format(record))

        # If this isn't a modify event, quit the loop early
        if record["eventName"] != "MODFIY":
            continue
