from models.httpResult import HttpResult
import requests

from requester.audio import download_wav

def send(url: str, method: str, data: str):
    if method == "GET":
        return get(url)
    elif method == "POST":
        return post(url, data)

def format_result(res: requests.Response):
    content_type = res.headers['Content-Type']
    data = HttpResult(
        content_type=content_type,
        status_code=res.status_code,
        body=None,
        time=res.elapsed.total_seconds()
    )
    
    if "application/json" in content_type:
        data.body = res.json()
        return data
    elif "audio" in content_type:
        download_wav(res)
        return data
    else:
        data.body = res.text
        return data

def format_exception(e: Exception):
    return HttpResult(
        content_type="-",
        status_code=999,
        body=e,
        time=0
    )

def get(url: str):
    try:
        res = requests.get(url=url)
    except Exception as e:
        return format_exception(e)
    return format_result(res)

def post(url: str, data: str):
    headers = {'Content-Type': 'application/json'}
    try:
        res = requests.post(url=url, data=data, headers=headers)
    except Exception as e:
        return format_exception(e)
    return format_result(res)
