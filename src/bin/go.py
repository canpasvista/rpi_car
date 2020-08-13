#!/usr/bin/env python

import RPi.GPIO as gpio
import time
import os
import a as car
import json
import udp

usleep = lambda x : time.sleep(x/1000000.0)
msleep = lambda x : time.sleep(x/1000.0)

gpio.setmode(gpio.BOARD)

kPinCLK = 8
kPinCLK2= 10

gpio.setup(kPinCLK, gpio.OUT)
gpio.setup(kPinCLK2,gpio.OUT)

print "hello"

j = json.loads(json.dumps({'stop':1,'R':0,'L':0}))

while True:
    msg = udp.net_test()
    if msg == 0:
        ""
    else:
        try :
            j = json.loads(msg)
        except:
            pass
        else:
            print j
    if j['stop'] == 1:
         msleep(1.0)
    else:
         car.go(j['R'],j['L'])
#    if(os.path.isfile("/home/pi/tmp/go")):
#        car.go(0,600)
#    if(os.path.isfile("/home/pi/tmp/back")):
#        car.back(600,0)
#    if(os.path.isfile("/home/pi/tmp/right")):
#        car.right(200,600)
#    if(os.path.isfile("/home/pi/tmp/left")):
#        car.left(600,200)
    msleep(1.0)

udp.close()
