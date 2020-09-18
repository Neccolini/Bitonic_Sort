def sort(x:list,up=True)->list:
    """
    リストxの要素をupで指定された向きにソート
    up=True -> 昇順
    up=False -> 降順
    len(x) must be 2^i
    """
    if len(x)==1:
        return x

    else:
        mid_point=len(x)//2
        first=sort(x[:mid_point],True)
        second=sort(x[mid_point:],False)

        x1=first+second

        return _sub_sort(x1,up)

def _sub_sort(x:list,up:bool)->list:
    if len(x)==1:
        return x

    else:
        _compare_and_swap(x,up)
        mid_point=len(x)//2
        first=_sub_sort(x[:mid_point],up)
        second=_sub_sort(x[mid_point:],up)

        return first+second

def _compare_and_swap(x:list,up:bool)->list:
    mid_point=len(x)//2
    for i in range(mid_point):
        if (x[i]>x[mid_point+i])==up:
            x[i], x[mid_point+i]=x[mid_point+i],x[i]