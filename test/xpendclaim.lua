local log = function(...)
  local li = {}
  for _, v in ipairs({ ... }) do
    table.insert(li, cjson.encode(v))
  end
  redis.log(redis.LOG_NOTICE, unpack(li))
end

local concat = function(li, push)
  for i = 1, #push do
    table.insert(li, push[i])
  end
end

-- local strBin = function(str)
--   local li = {}
--   for i = 1, #str do
--     li[i] = string.byte(str, i)
--   end
--   return li
-- end

local intBin = function(n)
  local t = {}
  while n > 0 do
    local r = math.floor(math.fmod(n, 256))
    table.insert(t, r)
    n = (n - r) / 256
  end
  return t
end

local splitNum = function(str)
  local pos = string.find(str, "-") -- 找到"-"的位置
  local str1 = string.sub(str, 1, pos - 1) -- 截取前半段字符串
  local str2 = string.sub(str, pos + 1) -- 截取后半段字符串
  return { tonumber(str1), tonumber(str2) } -- 返回两个数字
end

local XPENDING = function(stream, group, idle, limit)
  return redis.call("XPENDING", stream, group, "IDLE", idle, "-", "+", limit)
end

local XCLAIM = function(stream, group, customer, min_idle, ...)
  return redis.call("XCLAIM", stream, group, customer, min_idle, ...)
end

local XINFO = function(stream, group)
  return redis.call("XINFO", "CONSUMERS", stream, group)
end

local XACK = function(stream, group, id)
  return redis.call("XACK", stream, group, id)
end

local XDEL = function(stream, id)
  return redis.call("XDEL", stream, id)
end

local XDELCONSUMER = function(stream, group, consumer)
  return redis.call("XGROUP", "DELCONSUMER", stream, group, consumer)
end

function xackdel(keys, args)
  local stream, group = unpack(keys)
  local id = args[1]
  XACK(stream, group, id)
  XDEL(stream, id)
end

function xconsumerclean(keys, args)
  local stream, group = unpack(keys)
  local expire = tonumber(args[1])
  for _, v in ipairs(XINFO(stream, group)) do
    local v = v.map
    if v.idle > expire then
      XDELCONSUMER(stream, group, v.name)
    end
  end
end

function xpendclaim(keys, args)
  if #keys ~= 3 then
    return
  end
  local stream, group, customer = unpack(keys)
  local idle, limit = unpack(args)
  idle = tonumber(idle)
  local li = XPENDING(stream, group, idle, limit)
  if #li > 0 then
    local id_li = {}
    local id_retry = {}
    --[[
https://redis.io/commands/xpending/
对于每条消息，将返回四个属性：

1 消息的 ID
2 获取消息但仍需确认消息的使用者的名称。我们称它为消息的当前所有者
3 自上次将此消息传递给此使用者以来经过的毫秒数
3 传递次数
    --]]
    for _, v in ipairs(li) do
      local id = v[1]
      table.insert(id_li, id)
      id_retry[id] = v[4]
    end
    local bin = {}
    local buf = ""
    for _, v in ipairs(XCLAIM(stream, group, customer, idle, unpack(id_li))) do
      local id, msg = unpack(v)
      table.insert(bin, id_retry[id])
      id = splitNum(id)
      concat(bin, id)
      for _, v in ipairs(msg) do
        table.insert(bin, #v)
        -- log(">", strBin(v))
        buf = buf .. v
      end
    end
    bin = cmsgpack.pack(bin)
    local r = intBin(#bin)
    table.insert(r, 1, #r)
    return string.char(unpack(r)) .. bin .. buf
  else
    return
  end
end
