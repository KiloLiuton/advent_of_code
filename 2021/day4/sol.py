import re

with open('input', 'r') as f:
    data = [ l.split() for l in f.read().split('\n\n') ]

for i in range(len(data)):
    p = {}
    for x in data[i]:
        key, val = x.split(':')
        p[key] = val
    data[i] = p

required_fields = ( 'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid' )
valid_passports = []
for passport in data:
    if all([ (k in passport.keys()) for k in required_fields ]):
        valid_passports.append(passport)
print('Valid passports:', len(valid_passports))

# part 2

def monadicPipe(*fs):
    def pipe(x):
        for f in fs:
            if x is None: return None
            x = f(x)
        return x
    return pipe

def haslength(n):
    return lambda s: s if len(s) == n else None

def try_(f):
    def g(x):
        try: return f(x)
        except: return None
    return g

def isbetween(a, A):
    def g(x):
        if x>a and x<A: return x
        else: return None
    return g

def matchpat(pat):
    return re.compile(pat).match

def check_hgt(match):
    if match is None: return None
    s = match.group(match.lastindex)
    if 'cm' in s and isbetween(150, 193)(int(s.strip('cm'))):
        return s
    elif 'in' in s and isbetween(59, 76)(int(s.strip('in'))):
        return s
    else:
        return None

def check_ecl(match):
    if match is None: return None
    eye_colors = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
    s = match.group(match.lastindex)
    if s in eye_colors:
        return match.string
    else:
        return None

byr = monadicPipe( haslength(4), try_(int), isbetween(1920, 2002) )
iyr = monadicPipe( haslength(4), try_(int), isbetween(2010, 2020) )
eyr = monadicPipe( haslength(4), try_(int), isbetween(2020, 2030) )
hgt = monadicPipe( matchpat('([0-9]{3}cm)|([0-9]{2}in)'), check_hgt )
hcl = monadicPipe( matchpat('#[0-9a-f]{6}') )
ecl = monadicPipe( matchpat('(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)'), check_ecl)
pid = monadicPipe( haslength(9), try_(int) )

valid_passports_new = []
for passport in valid_passports:
    isvalid = True
    if any([globals().get(key)(passport[key]) is None for key in required_fields]):
            isvalid = False
    if isvalid: valid_passports_new.append(passport)
print('new valid passports', len(valid_passports_new))
