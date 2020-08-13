from socket import socket, AF_INET, SOCK_DGRAM

HOST = ''   
PORT = 5000


s = socket(AF_INET, SOCK_DGRAM)
s.setblocking(0)

s.bind((HOST, PORT))



def net_test():
    try:
        msg, address = s.recvfrom(8192)
    except:
        return 0
    else:
        return msg

#while True:
#    net_test()
def close():
    s.close()
