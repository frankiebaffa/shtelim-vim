#!/bin/python3

import os
import glob
import re

def reformatContent(filename):
    newcontents = ""
    enc = 'iso-8859-15'
    with open(filename,'r',encoding=enc) as file:
        txt = file.read()
        lines = txt.split('\n')
        for line in lines:
            stamppat   = r"^[a-zA-Z\-]+\s[a-zA-Z\-]+\s\([0-9]{1,2}\/[0-9]{1,2}\/[0-9]{4}\s[0-9]{1,2}:[0-9]{1,2}:[0-9]{1,2}\s[AP]M\):\s"
            stampmatch = re.findall(stamppat,line)
            hasstamp   = len(stampmatch) > 0

            statuspat   = r"[A-Za-z]+\s[A-Za-z]+\shas"
            statusmatch = re.findall(statuspat,line)
            hasstatus   = len(statusmatch) > 0
            if hasstamp:
                stamp = stampmatch[0]
                stamp = stamp.rstrip()
                newcontents += f"{stamp}\n"
                text = line.replace(stamp,'')
                while text[0:4] != "    ":
                    text = f" {text}"
                text = text.rstrip()
                if len(text) > 70:
                    count = 0
                    while text != '':
                        if (count == 2000):
                            print("Broken due to high repetition")
                            print(text)
                            break
                        if len(text) > 70:
                            spacebreaki =  text[0:70].rfind(' ')
                            if spacebreaki <= 3:
                                spacebreaki = text[4:len(text)].find(' ')
                                newline = None
                                if spacebreaki >= len(text) - 1:
                                    newline = text
                                else:
                                    newline = text[0:spacebreaki]
                                text = text.replace(newline,'')
                                newline = newline.rstrip()
                                newcontents += f"{newline}\n"
                            else:
                                newline     =  text[0:spacebreaki]
                                newcontents     += f"{newline}\n"
                                text        =  text.replace(newline,'')
                                if len(text) > 0:
                                    while text[0:4] != "    ":
                                        text = f" {text}"
                        else:
                            text = text.rstrip()
                            newcontents += f"{text}\n"
                            text = text.replace(text,'')
                        count += 1
                else:
                    text = text.rstrip()
                    newcontents += f"{text}\n"
            if hasstatus:
                status = line
                status = status.rstrip()
                newcontents += f"{status}\n"
    with open(filename,'w') as file:
        file.write(newcontents)

files = glob.glob('*.txt')
for file in files:
    try:
        firstUn = file.index('__')
        fname = file[0:firstUn]
        secondUn = file[firstUn+2:len(file)].index('_')
        lname = file[firstUn+2:secondUn+firstUn+2]
        dtStart = file.index(lname)+len(lname)+1
        monthEnd = file[dtStart:len(file)].index('_')+dtStart
        month = file[dtStart:monthEnd]
        if(len(month)==1):
            month = '0'+month
        dayEnd = file[monthEnd+1:len(file)].index('_')+monthEnd+1
        day = file[monthEnd+1:dayEnd]
        if(len(day)==1):
            day = '0'+day
        yearEnd = file[dayEnd+1:len(file)].index('__')+dayEnd+1
        year = file[dayEnd+1:yearEnd]
        timeStart = yearEnd+2
        hourEnd = file[timeStart:len(file)].index('_')+timeStart
        hour = file[timeStart:hourEnd]
        if(len(hour)==1):
            hour = '0'+hour
        minuteEnd = file[hourEnd+1:len(file)].index('_')+hourEnd+1
        minute = file[hourEnd+1:minuteEnd]
        if(len(minute)==1):
            minute = '0'+minute
        secondEnd = file[minuteEnd+1:len(file)].index('_')+minuteEnd+1
        second = file[minuteEnd+1:secondEnd]
        if(len(second)==1):
            second = '0'+second
        timePart = file[secondEnd+2:secondEnd+4]
        if(timePart=='PM'):
            hour = str(int(hour)+12)
            if(hour=='24'):
                hour = '00'

        newFile = '{}{}{}.{}{}{}.{}.{}.shtelim'.format(year,month,day,hour,minute,second,lname,fname)
        os.rename(file,newFile)
        print('file: '+file+' -> '+newFile)
        reformatContent(newFile)
        print('file: '+file+': reformatted')
        print('')
    except ValueError:
        pass
