import time
from pywinauto.application import Application
from pywinauto import application
import pywinauto.keyboard
import pywinauto.mouse
import pywinauto.base_wrapper
import pywinauto.timings
import os
app = application.Application()

try:    app.start('C:\\Program Files\\Mozilla Firefox\\firefox.exe')
except: # Failed
    a = "Firefox failed to open!"
else:  # Success
    time.sleep(1)
    pywinauto.mouse.click(button='left', coords=(200, 60))
    pywinauto.keyboard.send_keys("youtube.com") # adds first tab
    pywinauto.keyboard.send_keys("{ENTER}")
    pywinauto.mouse.click(button='left', coords=(245, 20))# clicks new tab button
    pywinauto.mouse.click(button='left', coords=(200, 60))
    pywinauto.keyboard.send_keys("twitter.com") # adds second tab
    pywinauto.keyboard.send_keys("{ENTER}")

app = application.Application()
time.sleep(1)
try:    app.start('C:\\Program Files\\KeePassXC\\KeePassXC.exe')
except: # Failed
    a = "KeePassXC failed to open!"
else:  # Success

    time.sleep(2)
    pywinauto.mouse.click(button='left', coords=(700, 430))
    time.sleep(2)
    pywinauto.keyboard.send_keys("passoword") # type password
    pywinauto.mouse.click(button='left', coords=(1100, 590))

try:    app.start('C:\\Users\\davon\\AppData\\Roaming\\Spotify\\Spotify.exe')
except: # Failed
    a = ""
else:  # Success
    a = ""

try:    app.start('C:\\Users\\davon\\AppData\\Local\\Discord\\Update.exe --processStart Discord.exe')
except: # Failed
    a = ""
else:  # Success
    a = ""