import json


def evaluate(evidence, evaluations, metadata):
    try:
        document = json.loads(evidence.decode("utf-8", errors="strict"))
    except (UnicodeDecodeError, json.JSONDecodeError):
        return {
            "conclusion": "inconclusive",
            "facts": [],
            "reason": "The application configuration could not be read as JSON.",
        }

    observed = "object" if isinstance(document, dict) else "other"
    return {
        "conclusion": "true" if observed == "object" else "false",
        "facts": [
            {
                "name": "configuration_structure",
                "value": observed,
                "value_type": "text",
                "status": "found",
            }
        ],
        "reason": "The parsed application configuration structure was evaluated.",
    }
