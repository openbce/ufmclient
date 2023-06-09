# ufmclient
The rust client of UFM

## Example
### Set environment
```
export UFM_ADDRESS=$ufm_server_address
export UFM_ADDRESS=$ufm_server_token 
```
### Version
```
./ufm version
6.11.1-2
```
### Create a Partition Key
```
./ufm create --pkey 5 --mtu 2 --membership full --service-level 0 --rate-limit 2.5 --guids 0011223344560200 --guids 1070fd0300176625 --guids 0011223344560201
```

### View a Partition Key
```
./ufm view --pkey 0x5
Name           : api_pkey_0x5
Pkey           : 0x5
IPoIB          : false
MTU            : 2
Rate Limit     : 2.5
Service Level  : 0
Ports          : 
    GUID                ParentGUID          PortType  SystemID            LID       SystemName     LogState  Name                
    0011223344560200    1070fd0300176624    virtual   1070fd0300176624    7         hpc-cloud01    Active                        
    1070fd0300176625                        physical  1070fd0300176624    4         hpc-cloud01    Active    1070fd0300176625_2  
    0011223344560201                                                      65535                    Unknown  

```

### List Partition Keys
```
./ufm list
Name           Pkey      IPoIB     MTU       Rate      Level     
api_pkey_0x5   0x5       false     2         2.5       0         
api_pkey_0x2   0x2       false     2         2.5       0         
management     0x7fff    true      2         2.5       0         
api_pkey_0x1   0x1       false     2         2.5       0         
api_pkey_0x4   0x4       false     2         2.5       0  
```

### Delete a Partition Key
```
./ufm delete --pkey 0x2
```