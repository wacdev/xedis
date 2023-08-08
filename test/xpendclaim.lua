local log = function(...)
  local li = {}
  for _, v in ipairs({ ... }) do
    table.insert(li, cjson.encode(v))
  end
  redis.log(redis.LOG_NOTICE, unpack(li))
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

local XDELCONSUMER = function(stream, group, consumer)
  return redis.call("XGROUP", "DELCONSUMER", stream, group, consumer)
end

function xconsumerclean(keys, args)
  local stream, group = unpack(keys)
  local expire = 1000 * tonumber(args[1])
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

    local r = {}
    for _, v in ipairs(XCLAIM(stream, group, customer, idle, unpack(id_li))) do
      local id, msg = unpack(v)

      for i, v in ipairs(msg) do
        msg[i] = cmsgpack.unpack(v)
      end

      table.insert(msg, 1, id_retry[id])
      table.insert(msg, 1, id)

      table.insert(r, msg)
    end
    return cmsgpack.pack(r)
  else
    return
  end
end
