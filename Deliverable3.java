import static org.junit.Assert.*;
import org.junit.Before;
import org.junit.Test;
import org.openqa.selenium.*;
import org.openqa.selenium.firefox.FirefoxDriver;
import org.openqa.selenium.htmlunit.HtmlUnitDriver;



public class Deliverable3 {

	static WebDriver driver = new HtmlUnitDriver();
	
	// Start at the home page for hoodopper
	@Before
	public void setUp() throws Exception {
		driver.get("http://lit-bayou-7912.herokuapp.com/");
	}
	
	
	/**
	 * As a user,
	 * I would like to have a correct display that is simple,
	 * So that I can view the output quickly
	 */
	
	// Given that I am on the main page
	// When I view the title
	// Then I see that it contains the word "Hoodpopper"
	@Test
	public void testCorrectTitle() {
		
		// Check that the title contains "Hoodpopper"
		
		String title = driver.getTitle();
		assertTrue(title.contains("Hoodpopper"));
	}
	
	//Given that I am on the main page
	//When I view the title
	//Then the title should exist
	@Test
	public void testTitleExists(){
		
		assertNotNull(driver.getTitle());
	}
	//Given that I am on the main page
	//When I view the page
	//Then a header should exist
	@Test
	public void testHeaderExists(){
		
		assertNotNull(driver.findElement(By.tagName("head")));
	}
	//Given that I am on the main page
	//When I click parse
	//Then a header should exist
	@Test
	public void testHeaderParse(){
		
		driver.findElement(By.xpath("(//input[@name='commit'])[2]")).click();
		String output = driver.findElement(By.tagName("head")).getText();
		assertNotNull(output);
	}
	//Given that I am on the main page
	//When I click compile
	//Then a header should exist
	@Test
	public void testHeaderCompile(){
		
		driver.findElement(By.xpath("(//input[@name='commit'])[3]")).click();
		String output = driver.findElement(By.tagName("head")).getText();
		assertNotNull(output);
	}
	
	//Given that I am on the main page
	//When I click tokenize
	//Then a header should exist
	@Test
	public void testHeaderToken(){
		
		driver.findElement(By.name("commit")).click();
		String output = driver.findElement(By.tagName("head")).getText();
		assertNotNull(output);
	}
	//Given that I am on the main page
	//When I view the page
	//Then I should see a text box
	@Test
	public void testTextBoxExists(){
		
		boolean display = driver.findElement(By.id("code_code")).isDisplayed();
		assertEquals(true,display);
	
	}
	
	// Given that I am on the main page
	// When I type Meow into the textbox
	// Then I should see Meow in the textbox
	@Test
	public void testTextinBox() {
		
		// Check that that the text box can accept input
		
		try {
			WebElement e = driver.findElement(By.id("code_code"));
			e.sendKeys("Meow");
			String elementText = e.getText();
			assertTrue(elementText.contains("Meow"));
		} catch (NoSuchElementException nseex) {
			fail();
		}
	}
	
	
	/**
	 * As a user,
	 * I would like to have the three compiler options,
	 * So that I can see the steps of Ruby code being compiled
	 */
	
	
	// Given that I am on the main page
	// When I view the page
	// Then I see that it contains tokenize button
	@Test
	public void testTokenizeButton(){
		
		WebElement tokenizeButton = driver.findElement(By.name("commit"));
		assertNotNull(tokenizeButton);
	}
	// Given that I am on the main page
    // When I view the page
	// Then I see that it contains parse button
	@Test
	public void testParseButton(){
			
		WebElement parseButton = driver.findElement(By.xpath("(//input[@name='commit'])[2]"));
		assertNotNull(parseButton);
	}
	
	// Given that I am on the main page
    // When I view the page
	// Then I see that it contains compile button
	@Test
	public void testCompileButton(){
			
		WebElement compileButton = driver.findElement(By.xpath("(//input[@name='commit'])[3]"));
		assertNotNull(compileButton);
	}
	
	/**
	 * As a user,
	 * I would like to have navigation options,
	 * So that I can can go back to previous pages
	 */
	
	// Given that I am on the main page
	// When I click the Compile button
	// Then I see a back button
	@Test
	public void testBackButtonLinkCompile() {
		
		// Check for back button after clicking compile
		driver.findElement(By.xpath("(//input[@name='commit'])[3]")).click();
		try {
			String name = driver.findElement(By.tagName("a")).getText();
			assertEquals(name, "Back");
		} catch (NoSuchElementException nseex) {
			fail();
		}
	}
	// Given that I am on the main page
	// When I click the Parse button
	// Then I see a back button
	@Test
	public void testBackButtonLinkParse() {
		
		// Check for back button after clicking Parse
		driver.findElement(By.xpath("(//input[@name='commit'])[2]")).click();
		try {
			String name = driver.findElement(By.tagName("a")).getText();
			assertEquals(name, "Back");
		} catch (NoSuchElementException nseex) {
			fail();
		}
	}
	
	// Given that I am on the main page
	// When I click the Parse button
	// Then I see a back button
	@Test
	public void testBackButtonLinkToken() {
		
		// Check for back button after clicking Tokenize
		driver.findElement(By.name("commit")).click();
		try {
			String name = driver.findElement(By.tagName("a")).getText();
			assertEquals(name, "Back");
		} catch (NoSuchElementException nseex) {
			fail();
		}
	}
	


}