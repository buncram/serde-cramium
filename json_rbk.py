import json
import pprint

hash_json = '{"dest":[1,2,3,4,5,6,7,8],"src1":[1,2,3,4],"src2":null,"param":{"Hash":["Off","On","Keccak",{"msg":[]}]}}'
hash_cmd = json.loads(hash_json)
pprint.pp(hash_cmd)
print(hash_cmd['param'])