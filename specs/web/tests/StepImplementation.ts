import { chromium, expect } from "@playwright/test";
import type { BrowserContext, Page } from "@playwright/test";
import { AfterScenario, BeforeScenario, Step } from "gauge-ts";

export default class StepImplementation {
  private page: Page;
  private context: BrowserContext;

  @BeforeScenario()
  public async beforeScenario() {
    const browser = await chromium.launch({ headless: true });
    this.context = await browser.newContext();
    this.page = await this.context.newPage();
  }

  @AfterScenario()
  public async afterScenario() {
    await this.context.close();
    await this.page.close();
  }

  @Step("<uri> にアクセスする。")
  public async open(uri: string) {
    await this.page.goto(`http://localhost:3000${uri}`);
    await expect(
      this.page.getByText("This page could not be found.")
    ).not.toBeVisible();
  }

  @Step("<link> リンクをクリックする。")
  public async clickLink(link: string) {
    const linkElement = this.page.locator(`[aria-label="${link}"]`);
    await linkElement.click();
  }

  @Step("<millisec> ミリ秒待機する。")
  public async wait(millisec: string) {
    await this.page.waitForTimeout(+millisec);
  }

  @Step("<inputArea> 入力欄が活性であることを確認する。")
  public async verifyInputAreaEnabled(inputArea: string) {
    const input = this.page.locator(`[aria-label="${inputArea}"]`);
    await expect(input).toBeVisible();
    await expect(input).toBeEnabled();
  }

  @Step("<inputArea> 入力欄に <inputValue> を入力する。")
  public async input(inputArea: string, inputValue: string) {
    const input = this.page.locator(`[aria-label="${inputArea}"]`);
    await input.fill(inputValue);
  }

  @Step("<buttonText> ボタンが活性であることを確認する。")
  public async verifyButtonEnabled(buttonText: string) {
    const button = this.page.locator(`button:has-text("${buttonText}")`);
    await expect(button).toBeVisible();
    await expect(button).toBeEnabled();
  }

  @Step("<buttonText> ボタンを押下する。")
  public async clickButton(buttonText: string) {
    const button = this.page.locator(`button:has-text("${buttonText}")`);
    await button.click();
  }

  @Step(
    "<table> 表の <row> 行目, <column> 列目に <value> が表示されていることを確認する。"
  )
  public async verifyTableValue(
    table: string,
    row: number,
    column: number,
    value: string
  ) {
    const cell = this.page
      .locator(`[aria-label="${table}"]`)
      .locator("tbody > tr")
      .nth(row - 1) // 2行目（0-indexed）
      .locator("td")
      .nth(column - 1); // 1列目（0-indexed）
    const text = await cell.textContent();
    await expect(text).toBe(value);
  }
}
