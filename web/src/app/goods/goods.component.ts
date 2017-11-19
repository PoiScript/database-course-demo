import { Component, OnInit } from '@angular/core'

import { ApiService } from '../core'
import { JoinedGoods } from '../shared'

@Component({
  templateUrl: './goods.component.html'
})
export class GoodsComponent implements OnInit {

  goodsArray: JoinedGoods[]

  constructor (private api: ApiService) { }

  ngOnInit () {
    this.api.getGoods()
      .subscribe(goods => {
        this.goodsArray = goods
      })

  }

}
