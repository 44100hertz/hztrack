local sb, sc = string.byte, string.char
local map = {}
local remap = {
  O = '0', o = '0',
  I = '1', l = '1', i = '1',
  Z = '2', z = '2',
  S = '5', s = '5',
}
for i,v in pairs(remap) do map[sb(i)] = sb(v) end
for i = sb('a'), sb('z') do map[i] = map[i] or i + sb('A') - sb('a') end
for i = sb('A'), sb('Z') do map[i] = map[i] or i end
for i = sb('0'), sb('9') do map[i] = map[i] or i end
for i = 0, 127 do map[i] = map[i] or 0 end

print("const BASE32: [u8; 128] = {")
for i=0,127 do
  if map[i] == 0 then
    io.write("0,    ")
  else
    io.write(("b'%c', "):format(map[i]))
  end
  if i%8==7 then print() end
end
print("};")

local mapping = "0123456789ABCDEFGHJKLMNPQRTUVWXY"
local count = 1
for c in mapping:gmatch(".") do
  count = count + 1
end
print(count)
