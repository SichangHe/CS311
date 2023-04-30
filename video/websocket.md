<!-- slide -->
# WebSocket

![WebSocket logo](https://seeklogo.com/images/W/websocket-logo-91B815D333-seeklogo.com.png)

A communication protocol on the application layer, similar to HTTP.

<!-- slide -->
<div class="columns">

![WebSocket logo](https://seeklogo.com/images/W/websocket-logo-91B815D333-seeklogo.com.png)

- Main idea
- Features
- Usage
- Drawbacks
- Future

</div>

<!-- slide -->
## Main idea

TCPConnection in HTML5 spec
([link](https://www.w3.org/TR/2008/WD-html5-20080610/comms.html#tcp-connections))

<div class="columns">

![HTML5 logo](https://upload.wikimedia.org/wikipedia/commons/thumb/3/38/HTML5_Badge.svg/2048px-HTML5_Badge.svg.png)

<div class="flex-vert-center">

➕
</div>

<div class="flex-vert-center">

**T**ransmission **C**ontrol **P**rotocol
</div>
</div>

<!-- slide -->
## Features

- <!-- .fragment -->
    HTTP compatibility
    ![HTTP icon](https://pic.onlinewebfonts.com/svg/img_139564.png)
    - Same ports as HTTP: bypass most firewalls
    ![Firewall icon](https://i0.wp.com/pureinfotech.com/wp-content/uploads/2010/09/win-firewall-1.png)
- <!-- .fragment -->Low overhead 🐁
    - Single TCP connection ⏚
    - Small header
        ![Header icon](https://static.thenounproject.com/png/4154773-200.png)
- <!-- .fragment -->Full-duplex communication ↹
    - Server push
        ![Server icon](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAOEAAADhCAMAAAAJbSJIAAAAclBMVEX///8AAAAeHh709PSenp6lpaVra2t0dHTIyMjPz8/o6Og+Pj7i4uJLS0u0tLRubm67u7vCwsKOjo59fX2GhoZRUVHx8fHY2NhjY2OUlJSsrKzW1tZbW1s4ODgpKSlOTk4NDQ0VFRUlJSUxMTFFRUUbGxvMXE7eAAAI+UlEQVR4nO1d2WKrIBCNNYvZF7OYNEmz3f//xduEQdmSdBTqYDlPrQpyOAgDM5BWKyAgICAgICAgICAgICAgICAggCLiSTq+fNSFyzidxE75jbpR/eiO3BFM6yYHSB3xi691M8txddJU41PdvAScXFCko+AdB/sEF3VzUrCwTXCQZ91LBnFdGCS9vBwDywynkO/FdsZYDC5QkqnlfCHbD7vZlsKHExG5hA4H2x9j5ELEgZuWURJdByJOnTSMsnBQ37QkdCGiLGGcDl1CGOhmK+Xekpky1mtczjD+8aBcElf+3q3hJqNoW0RZQvfGza71vCoXhjqvDCW7lXOGMDnqm+6t2D27Iiod6dT0YqvYsheNTPemxlqvBp5ZF/7fOGe4hjeNDfc2cM+miNpYOHfL7/yZv3qv3Zyr9W5BRFNWA5cQXx4/v2dPRFrmTAFrIqpfIR3YEpGqhNZEpGaRirAjIl0JLdU+ZQntiEhZQiv1T1tCGyLSltCCAtQlrC4idQkra0Bfwqoi0pewogo+SFhNBh8krKSDHxJW+RL9kLCCEr5IWF5EXyQsrQXdqb2OciIuvZGwUGOJSuWRhIWImDR9jyQsRPx8/2iONkuyclYouwBXShuRBHTfOSuTXezw39SQJdm8f5IEwJUyRCSBgA63oZz2AJ5GTCgMMJz2/MC0NEPPEBgGhvQRGP5JhvPED8xLM2z+iB8YUkFgaEBgSAwNYbjpLJfbmbFIjWDY4buu9obtAg1gmG8juUPfD2SRoU65zCNoKAGfe+0Nthj2z+qy6zKK/vXFC6ODuiB0X7abiBfWqyj6WrcQWEcK1B2WthiymuwJV9g2K+HDYK+6CY+wdcm5cOX8uIIojCGSti8/YIthV70IGQsuEYiuF9bo2AVht+CMXZFkfY083nvfO0ZafkJBKjOEqiwkg0Dsr+IR2CgsSAZFKi502IV89Xa0XRiR5CkyliK7/51EWh3+AkNhZRzPcKe1QI6unMWY/bNRKog8Q60TETBjj4BXgjsl9oWgXjCcvGAIfdqnnENbVNQDhrMXDNNGMOT3TYAhAVop73luj/9klzZphsatP9ITkMWR/QOid36VYbXRojWbGFE4Z2H79mFUpFZct7YYpmphofiCDdVRK6HFpgTCZ5NolfAO+f6n04H/pbhubTFcK5XPTRjRxnxcEA1T1qrEAfrxIaGORND3kikzKGuW92AfHWVra3eMMnkbTy86y6PxfBiNZQtkiQ2baJ0Vgh3lvv/zw7V86Ig2QfSfobyLTTfam8Cw1edbVxeGQjWC4TeNZLebmc90KMHwaOyyyALGkyMiyf5Zi6cJsN+19ZsXgLk6JkClTkD4zxaRhFuLfojIp2D9948WiMokqgm58Y5K1eapMMrXg/xsCUzgXkuct3XbHbpoC4cAImvm+byNKtAflOlgEcoo8Tn5RbFMf9F4hu33uZICsif9Cz1NMefMtm262GZ5OU84gnkbtX4Ao3XkpzqpaxuvUVb6OsD9UaV2lDTX8oaRwsExqE4A66iYEaP5M+Dmr2JQXIlq9SeTubWVKHoM+3zF1PSxNYFhMbDn3m8B/jOMz5J9pslozzPTPR3k7nVyOHVlz0x6Pso2cDKO9rJnZvHxgbSU1K0DrjwzuneN2Xaid+1R2WLYFAujFyk+HK2odcpepELZ21qrh5S1L8FNXMFDGvGQKHXkI+7lns+MWKuZXu8Ngc8I3Hi5nUQqfGotkEOJVLiwf8B8+d1Ihd+NNmEr+P5EmzQ/Yqj5UV8/iNyDt/ABh1k3/kTu5SFAOjI5C3gLRF/KZg3t0WKzWBpR5AE26SNKhy8+uImg1aKgoYm5joLO12G6KQ+KuspPuItkn6p8GGfRJGMdgzgdOCmivsdNa8GOItkflSlvBLi3ykS8MLqovcBdVul8jfV3gYfVdiP0lAcszp70gmlX9BmX/gh2VvZrO0rqw+gsENSD4hrAUFiEXxkOXGkEw++Bop2mi4nxAy7BELaK+XGIUm6cXxBJIEwOMU2tFTDGYg59Wj7ptIhi/6wLeg7+8weY07PqAzflUE2O91yokbkm5AYBKhU/c8+DTzH/uQ1c8HjxKzbXdtKni6Rd/Cwjcmx7vmuOKtCHy1H41VgMShzxmL3PlRCy94Q0OP7BHMso0SP6RVCZpv4Eg/d5EgPWiD4USfdduhD20iADR/K16D318y83+e/A6b7hV+B+LHTEXw3gk+Pr+0cLcJcd/ai2O3hkGyY4Bqrl7KxQdnHGNzgY7XHRfvUBBMGM+mDOUu9lODb4DxFWomjNDuPR5smYBzNEv9fa1ltWpr3JPGvCaqIQV3/Vu8wGMPyS7DNNRnsM21kqV+AmzZQuepf1ZMfQYJkpW3dn3S7O/FAI6sHA1hj+UyvwMf+Q1mHH6rg00rqshxdV9R69hL63RSmYLYZgOghX2AXB4zzR+mBmwQsTcOjZEeuUhXMt3yKhVBBpP/53Mx4ZUaSAmr1+V0rMexy5ZKQZroVpmox8SsuUg+EcJuayk5w0Q/k8CAkwvEMj5fYVmxHKzZQyw1drJbDCu5FzYJ+6EP9BnOGPY6LyHptFZvgTE2X8dXFAR3qEp2d9jewVo8yQpzDgKmfBZ3Lnx3/y1Jx2TNRkNTaiGGXBomGGTCb+Y5shrGsVwzmsOAoTz4X2fu3d8OUhvAx8Vaw773f+aTV2hy2Gc1UxCFYS1mKZqifhkVRWLC8NxqjXT/lXYsas2aX3j3woXosPkbI6cq/vszRPvSsvBVJ96q3sDTYqwS/lAXtzizhR1zU2ifpMX7U4R4k6M+9j920qY4oWc9GA+aFkGIy12w1g2FoX7kxD3GYTGH5z7Exvw9XS6GIqz9CzmCgMQ4hZRfvkagJEEd/eP5kDbBPUWkONgK1f6jHfr8C7Zz+aKffmYjZm83AalMOqNvCQGlTHyAMxrvRVHHCCuHCMYt7WSwYxXQySYvslcm/983kbVWD6mQeO7/MkBcxxAwzx+0xJoYT9Ffuk4rGcgenPt4j+BjlGfkTvZVVOKIkn6e2DMm7phNIMKCAgICAgICAgICAgICAgICDgD+M/1U2d6M9S3PwAAAAASUVORK5CYII=)

<!-- slide -->
### Connection process

<!-- TODO: Detailed process -->

- <!-- .fragment -->Connection upgrade
- <!-- .fragment -->
- <!-- .fragment -->

<!-- slide -->
### WebSocket message header

<!-- TODO: Detailed header explanation -->

<!-- slide -->
## Usage

- <!-- .fragment -->Alternative to HTTP AJAX requests
    - E.g. LiveView vs Hotwire

<div class="columns fragment">

![Phoenix icon](https://static-00.iconduck.com/assets.00/phoenix-icon-512x350-fqz59vyh.png)

<div class="flex-vert-center"

VS
</div>

![Hotwire icon](https://callmarx.dev/assets/svg/hotwire-turbo.svg)
</div>

<!-- slide -->
## Drawbacks

- <!-- .fragment -->Stateful
    - Complexity on server
        ![Complexity icon](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSfMcTeumZPIMadVWToLsQLPnFX_sf7m_qMoA&usqp=CAU)
- <!-- .fragment -->Tricky deployment
    ([example](https://www.reddit.com/r/django/comments/10q78lz/comment/j7ldxna/?utm_source=share&utm_medium=web2x&context=3))

<!-- slide -->
## Future

- <!-- .fragment -->Increased adoption on server side
    - tRPC ![tRPC icon](https://avatars.githubusercontent.com/u/78011399?s=280&v=4)
        ([link](https://trpc.io/docs/subscriptions))
    - HTMX ![HTMX icon](https://styles.redditmedia.com/t5_2u59z4/styles/communityIcon_3wi5tbhd61181.png)
        ([link](https://htmx.org/docs/#websockets-and-sse))
- <!-- .fragment -->Framework do the heavy lifting
    ![Phoenix icon](https://static-00.iconduck.com/assets.00/phoenix-icon-512x350-fqz59vyh.png)
    - Hide complexity from framework users

<style>
.columns {
    display: grid;
    gap: 2rem;
    grid-auto-flow: column;
}

.flex-vert-center {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    justify-content: center;
}

.reveal img {
    margin: 0 0;
}

.reveal li > img {
    height: 35px;
}
</style>
