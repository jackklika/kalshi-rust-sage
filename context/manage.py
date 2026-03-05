"""
Operations to populate and update LLM context
"""

from collections.abc import Collection
from dataclasses import dataclass
from pathlib import Path
import urllib.request

# current directory
CONTEXT_ROOT = Path(__file__).parent

@dataclass(frozen=True)
class ContextFile:
    filename: str
    url: str

CONTEXTS = [
    ContextFile("kalshi_openapi.yaml", "https://docs.kalshi.com/openapi.yaml"),
    ContextFile("kalshi_asyncapi.yaml", "https://docs.kalshi.com/asyncapi.yaml"),
    ContextFile(
        "polymarket_clob_openapi.yaml",
        "https://docs.polymarket.com/api-reference/clob-subset-openapi.yaml",
    ),
]

def refresh_context(contexts: Collection[ContextFile]) -> None:
    """Fetch all context and store in local folder."""
    print(f"Refreshing contexts in {CONTEXT_ROOT}...")

    for context in contexts:
        destination = CONTEXT_ROOT / context.filename
        print(f"Fetching {context.url} -> {destination}")

        try:
            with urllib.request.urlopen(context.url) as response:
                if response.status != 200:
                    print(f"Failed to download {context.url}: Status {response.status}")
                    continue

                content = response.read()

                with open(destination, "wb") as f:
                    f.write(content)

            print(f"Successfully saved {context.filename}")

        except Exception as e:
            print(f"Error fetching {context.url}: {e}")


if __name__ == "__main__":
    refresh_context(CONTEXTS)
