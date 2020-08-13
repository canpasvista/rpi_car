#!/usr/bin/env python

import RPi.GPIO as gpio
import time
import os

usleep = lambda x : time.sleep(x/1000000.0)
msleep = lambda x : time.sleep(x/1000.0)

gpio.setmode(gpio.BOARD)

kPinCLK = 8
kPinCLK2= 10

gpio.setup(kPinCLK, gpio.OUT)
gpio.setup(kPinCLK2,gpio.OUT)

print "hello"

def on():
    gpio.output(kPinCLK, True)
    gpio.output(kPinCLK2, True)
    usleep(1000.0)

def speed(L,R):
    L += 50
    R += 50
    count = 0
    while 1:
        if count >= L :
            gpio.output(kPinCLK, False)
            L+=1000
            break
        if count >= R :
            gpio.output(kPinCLK2, False)
            R+=1000
            break
        count += 1
    while 1:
        if count >= L :
            gpio.output(kPinCLK, False)
            break
        if count >= R :
            gpio.output(kPinCLK2, False)
            break
        count += 1
    usleep(1.0)
        
def go(x,y):
    on()
    speed(x,y)
    msleep(20.0)

def back(x,y):
    on()
    speed(x,y)
    msleep(20.0)

def right(x,y):
    on()
    speed(x,y)
    msleep(20.0)

def left(x,y):
    on()
    speed(x,y)
    msleep(20.0)

def test():
    a = 0
    b = 0
    while True:
        a += 10
        b += 10
        print a,"-",b
        for i in range(10):
            go(a,b)
            msleep(1.0)

# test()
