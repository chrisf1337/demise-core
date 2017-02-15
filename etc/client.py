import socket
import json
import time

HOST = 'localhost'
IN_PORT = 8766
OUT_PORT = 8765

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect((HOST, OUT_PORT))

sock.send(json.dumps({
    'clientId': '1234',
    'method': 'connect'
}).encode('utf-8'))
