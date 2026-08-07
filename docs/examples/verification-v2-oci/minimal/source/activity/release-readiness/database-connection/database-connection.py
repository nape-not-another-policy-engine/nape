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
    if not isinstance(endpoint, str):
        return {
            "conclusion": "inconclusive",
            "facts": [],
            "reason": "The database endpoint fact was not found as text.",
        }

    expected = "postgresql://orders-db.prod.acme.example:5432/orders"
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
        "reason": "The observed database endpoint was compared with the approved endpoint.",
    }
