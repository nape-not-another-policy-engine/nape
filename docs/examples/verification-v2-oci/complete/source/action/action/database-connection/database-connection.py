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

    database = document.get("database") if isinstance(document, dict) else None
    endpoint = database.get("endpoint") if isinstance(database, dict) else None
    evaluation = None
    for item in evaluations:
        if isinstance(item, dict) and item.get("name") == "approved-endpoint":
            evaluation = item
    criteria = evaluation.get("criteria") if isinstance(evaluation, dict) else None
    expected = criteria.get("equals") if isinstance(criteria, dict) else None
    if not isinstance(endpoint, str) or not isinstance(expected, str):
        return {
            "conclusion": "inconclusive",
            "facts": [],
            "reason": "The observed or approved database endpoint could not be established.",
        }

    return {
        "conclusion": "true" if endpoint == expected else "false",
        "facts": [
            {
                "name": "database_endpoint",
                "value": endpoint,
                "value_type": "text",
                "status": "found",
            }
        ],
        "reason": "The observed database endpoint was compared with the effective approved endpoint.",
    }
