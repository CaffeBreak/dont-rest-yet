from prisma import Prisma


async def main() -> None:
  async with Prisma() as db:
    # 新しい投稿を作成
    post = await db.post.create({
        'title': 'My first post',
        'content': 'This is my first post using Prisma with Python!',
        'published': False,  # この行を追加
    })
    print(f'Created new post with id: {post.id}')

    # 投稿を取得
    posts = await db.post.find_many()
    print(f'Found {len(posts)} posts')

    # 投稿を更新
    updated_post = await db.post.update({
        'where': {'id': post.id},
        'data': {'published': True},
    })
    print(f'Updated post {updated_post.id}, published: {updated_post.published}')

    # 投稿を削除
    await db.post.delete({'where': {'id': post.id}})
    print(f'Deleted post with id: {post.id}')

if __name__ == '__main__':
  import asyncio
  asyncio.run(main())
