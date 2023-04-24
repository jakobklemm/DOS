detector = peripheral.wrap("right")
rednet.open("top")
from, mes = rednet.receive("tab")
x, y, z = gps.locate()
id = os.getComputerID()
pos = detector.getNearbyPlayers(128000)

res = id .. ":" .. x .. ":" .. y .. ":" .. z .. "\n"

for i = 1,#pos do
    p = pos[i].player .. ":" .. pos[i].distance
    res = res .. p .. "\n"
end

print(res)

http.post("https://metrics.thearmabois.com/submit/pos", res)