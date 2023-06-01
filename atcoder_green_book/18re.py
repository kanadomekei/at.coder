N=int(input())
A=list(map(int, input().split()))
mod=10**9+7

A_sum=sum(A)

ans=0

for i in range(N):
    A_sum-=A[i]
    ans+=A[i]*(A_sum)
    ans%=mod

print(ans)