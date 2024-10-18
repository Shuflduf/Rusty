import datetime
from os import listdir

log_file_path = None
print(str(datetime.datetime.now().time()))
def init():
    global log_file_path
    log_num = 0
    for i in listdir("Logs/"):
        num = int(i[3:].split('.')[0])
        if num > log_num:
            log_num = num
    log_file_path = f"Logs/log{log_num + 1}.txt"
    f = open(log_file_path, 'a')
    f.write(f"[{str(datetime.date.today())}]")
    f.write("\n")
    f.close()
    
def log_text(text):
    f = open(log_file_path, 'a')
    f.write(f"[{str(datetime.datetime.now().time())}] {text}")
    f.write("\n")
    f.close()
