import socket
import json

HOST = 'localhost'
IN_PORT = 8766
OUT_PORT = 8765

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect((HOST, OUT_PORT))

msg = json.dumps({
    'clientId': '1234',
    'method': 'connect'
}).encode('utf-8')

sock.send(len(msg).to_bytes(4, byteorder='little'))
sock.send(msg)

n_bytes = int.from_bytes(sock.recv(4), byteorder='little')
print(json.loads(sock.recv(n_bytes).decode('utf-8')))
