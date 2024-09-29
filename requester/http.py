import requests

def get(url: str):
    res = requests.get(url=url)
    content_type = res.headers['Content-Type']
    if not 'json' in content_type:
        return "Sorry, we don't still implement for except json."
    else:
        return res.json()
