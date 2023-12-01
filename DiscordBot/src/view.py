from cProfile import label
import enum
from multiprocessing import BufferTooShort
from turtle import title, update
from typing import Any, Callable, Coroutine, Optional, Union
from discord import ButtonStyle, Color, Embed, Interaction, SelectOption
import discord
from discord.ui import View, button, Button, select, Select
from const import JST

from pb.dry.reminder import Task,UpdateTaskRequest

class PaginationView[T](View):
  def __init__(
    self,
    datas: list[T],
    gen_embed: Callable[[list[T], int, int, Optional[str], Optional[Union[int, Color]]], Embed],
    title: Optional[str] = None,
    color: Optional[Union[int, Color]] = None
  ):
    super().__init__(timeout=None)
    
    self.title = title
    self.color = color
    self.page_list = [datas[i:i + 10] for i in range(0, len(datas), 10)]
    self.current_page = 0
    self.page_max = (len(datas) - 1) // 10
    self.gen_embed: Callable[[list[T], int], Embed] = lambda l, i: gen_embed(l, i, self.page_max, title, color)
    
    self.prev.disabled = True
    self.next.disabled = self.page_max == 0
      
    self.goto.label = f"{self.current_page + 1}/{self.page_max + 1}"
    
  def get_init_embed(self) -> Embed:
    return self.gen_embed(self.page_list[0], 0)
    
  @button(label="<", custom_id="prev")
  async def prev(self, interaction: Interaction, _: Button):
    self.current_page -= 1

    self.prev.disabled = (self.current_page == 0)
    self.next.disabled = False
    self.goto.label = f"{self.current_page + 1}/{self.page_max + 1}"
    
    embed = self.gen_embed(self.page_list[self.current_page], self.current_page)
    
    await interaction.response.edit_message(embed=embed, view=self)
    
  @button(disabled=True)
  async def goto(self, _i: Interaction, _b: Button):
    pass

  @button(label=">", custom_id="next")
  async def next(self, interaction: Interaction, _: Button):
    self.current_page += 1
    
    self.next.disabled = (self.current_page == len(self.page_list) - 1)
    self.prev.disabled = False
    self.goto.label = f"{self.current_page + 1}/{self.page_max + 1}"

    embed = self.gen_embed(self.page_list[self.current_page], self.current_page)
    
    await interaction.response.edit_message(embed=embed, view=self)

class DeletePaginationView[T](PaginationView[T]):
  def __init__(
    self,
    datas: list[T],
    gen_embed: Callable[[list[T], int, int, str | None, int | Color | None], Embed],
    gen_options: Callable[[list[T]], list[SelectOption]],
    delete_task: Callable[[str], Coroutine[Any, Any, Task]],
    title: str | None = None,
    color: int | Color | None = None
  ):
    super().__init__(datas, gen_embed, title, color)
    
    self.target = ""
    self.delete_task = delete_task
    self.gen_options = gen_options
    
  @button(label="<", custom_id="prev")
  async def prev(self, interaction: Interaction, button: Button):
    self.delete_target.options = self.gen_options(self.page_list[self.current_page - 1])
    
    await super().prev(interaction, button) # type: ignore これ型推論が悪くて、実は呼び出し可能
    
  @button(label=">", custom_id="next")
  async def next(self, interaction: Interaction, button: Button):
    self.delete_target.options = self.gen_options(self.page_list[self.current_page + 1])
    
    await super().next(interaction, button) # type: ignore これ型推論が悪くて、実は呼び出し可能
    
  @select(custom_id="delete_target", placeholder="削除対象を選択", row=1)
  async def delete_target(self, interaction: Interaction, select: Select):
    self.target = select.values[0]
    self.delete_target.placeholder = [option.label for option in select.options if option.value == self.target][0]

    await interaction.response.edit_message(view=self)
    
  @button(label="消しちゃう", style=ButtonStyle.danger, custom_id="delete", row=2)
  async def delete(self, interaction: Interaction, _: Button):
    deleted = await self.delete_task(self.target)
    
    await interaction.response.edit_message(content=f"{deleted.title} - {deleted.remind_at.astimezone(JST)}を削除しちゃった", embed=None, view=None)
    
class bottonView[T](View):
  def __init__(
    self,
    update_task: Callable[[],Coroutine[Any, Any, Task]],
    title: str,
    yes: str,
    no: str,
  ):
    super().__init__()
    
    self.title = title
    self.update_task = update_task
    
    self.pressedY.label = yes
    self.pressedN.label = no
    
  @button(custom_id="yes")
  async def pressedY(self, interaction: Interaction, _: Button):
    name = interaction.user.display_name
    await interaction.response.edit_message(content=f"{name}は『{self.title}』を完了しました！", view=None)
      
  @button(custom_id="no")
  async def pressedN(self, interaction: Interaction, _:Button):
    updated = await self.update_task()
    await interaction.response.edit_message(content=f"更新しました。{updated.remind_at.astimezone(JST).strftime('%Y-%m-%d %H:%M')}に再度通知します", view=None)
  

  
