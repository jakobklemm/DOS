me = peripheral.wrap("bottom")

function submit()
    items = me.listAvailableItems()
    body = ""
    for i = 1,#items do
        l = items[i].name .. ":" .. items[i].damage .. ":" .. items[i].count .. "\n"
        body = body .. l 
    end

    http.post("https://metrics.thearmabois.com/submit/me", body)
end

submit()