import json
import os
from typing import AsyncGenerator, List, Optional

import aiohttp
import pytest_asyncio

from . import Account, AttrDict, Bot, Client, DeltaChat, EventType, Message
from .rpc import Rpc


async def get_temp_credentials() -> dict:
    url = os.getenv("DCC_NEW_TMP_EMAIL")
    assert url, "Failed to get online account, DCC_NEW_TMP_EMAIL is not set"
    async with aiohttp.ClientSession() as session:
        async with session.post(url) as response:
            return json.loads(await response.text())


class ACFactory:
    def __init__(self, deltachat: DeltaChat) -> None:
        self.deltachat = deltachat

    async def get_unconfigured_account(self) -> Account:
        return await self.deltachat.add_account()

    async def get_unconfigured_bot(self) -> Bot:
        return Bot(await self.get_unconfigured_account())

    async def new_configured_account(self) -> Account:
        credentials = await get_temp_credentials()
        account = await self.get_unconfigured_account()
        assert not await account.is_configured()
        await account.set_config("addr", credentials["email"])
        await account.set_config("mail_pw", credentials["password"])
        await account.configure()
        assert await account.is_configured()
        return account

    async def new_configured_bot(self) -> Bot:
        credentials = await get_temp_credentials()
        bot = await self.get_unconfigured_bot()
        await bot.configure(credentials["email"], credentials["password"])
        return bot

    async def get_online_accounts(self, num: int) -> List[Account]:
        accounts = [await self.new_configured_account() for _ in range(num)]
        for account in accounts:
            await account.start_io()
        return accounts

    async def send_message(
        self,
        to_account: Account,
        from_account: Optional[Account] = None,
        text: Optional[str] = None,
        file: Optional[str] = None,
        group: Optional[str] = None,
    ) -> Message:
        if not from_account:
            from_account = (await self.get_online_accounts(1))[0]
        to_contact = await from_account.create_contact(
            await to_account.get_config("addr")
        )
        if group:
            to_chat = await from_account.create_group(group)
            await to_chat.add_contact(to_contact)
        else:
            to_chat = await to_contact.create_chat()
        return await to_chat.send_message(text=text, file=file)

    async def process_message(
        self,
        to_client: Client,
        from_account: Optional[Account] = None,
        text: Optional[str] = None,
        file: Optional[str] = None,
        group: Optional[str] = None,
    ) -> AttrDict:
        await self.send_message(
            to_account=to_client.account,
            from_account=from_account,
            text=text,
            file=file,
            group=group,
        )

        return await to_client.run_until(lambda e: e.type == EventType.INCOMING_MSG)


@pytest_asyncio.fixture
async def rpc(tmp_path) -> AsyncGenerator:
    rpc_server = Rpc(accounts_dir=str(tmp_path / "accounts"))
    async with rpc_server:
        yield rpc_server


@pytest_asyncio.fixture
async def acfactory(rpc) -> AsyncGenerator:
    yield ACFactory(DeltaChat(rpc))