from urllib.request import urlopen
from bs4 import BeautifulSoup

def create_scrapper(url=None, html=None):
    if url:
        html = urlopen(url)
        return BeautifulSoup(html, 'html.parser')
    return BeautifulSoup(str(html), 'html.parser')
def to_money(fraction, cents):
    return float(fraction) + ( float(cents) / 100 )

def scrapping_mercado_livre(url):
    bs = create_scrapper(url=url)
    
    # nome do produto
    nome_produto = bs.find('h1', attrs={'class':'ui-pdp-title'}).getText()
    
    # preço
    div_preco_html = bs.find('div', attrs={'class':['ui-pdp-container__row--price']})
    bs_preco = create_scrapper(html=div_preco_html)
    preco_fraction = bs_preco.find('span', attrs={'class':'price-tag-fraction'}).getText()
    preco_centavos = bs_preco.find('span', attrs={'class':'price-tag-cents'}).getText()
    preco_produto = to_money(preco_fraction, preco_centavos)
    
    print('{}\nvalor: R$ {}\n'.format(nome_produto, preco_produto))

if __name__ == '__main__':
    urls = [
        'https://produto.mercadolivre.com.br/MLB-1848222103-kit-diy-ups-carregador-solar-com-bateria-18650-saida-12v-nf-_JM#reco_item_pos=0&reco_backend=machinalis-homes-pdp-boos&reco_backend_type=function&reco_client=home_navigation-recommendations&reco_id=5f3f418d-1841-4144-b7f5-c566738606bb&c_id=/home/navigation-recommendations/element&c_element_order=1&c_uid=f5fd59d1-dde4-4d80-bb57-10e3c0fb5c45'
        ,'https://produto.mercadolivre.com.br/MLB-1328065683-placa-modulo-carregador-bateria-litium-18650-usb-pcb-diy-_JM#reco_item_pos=2&reco_backend=machinalis-homes-pdp-boos&reco_backend_type=function&reco_client=home_navigation-recommendations&reco_id=5f3f418d-1841-4144-b7f5-c566738606bb&c_id=/home/navigation-recommendations/element&c_element_order=3&c_uid=4720d63c-1627-4f83-9fae-e33d3faaad05'
        ,'https://produto.mercadolivre.com.br/MLB-945048791-kit-bomba-dagua-com-painel-solar-bateria-e-controlador-_JM#reco_item_pos=3&reco_backend=machinalis-homes-pdp-boos&reco_backend_type=function&reco_client=home_navigation-trend-recommendations&reco_id=aeb505a5-a840-4908-b3da-189a1485b6c4&c_id=/home/navigation-trends-recommendations/element&c_element_order=4&c_uid=e8c232a1-0da1-4695-9681-2e3ef925f073'
        ,'https://produto.mercadolivre.com.br/MLB-775001531-kit-120-capacitores-12-valores-10-de-cada-conforme-lista-_JM#reco_item_pos=4&reco_backend=machinalis-homes-pdp-boos&reco_backend_type=function&reco_client=home_navigation-trend-recommendations&reco_id=aeb505a5-a840-4908-b3da-189a1485b6c4&c_id=/home/navigation-trends-recommendations/element&c_element_order=5&c_uid=f2e3a73d-9628-42bd-92f3-5ec9f16b3a5c'
        ,'https://produto.mercadolivre.com.br/MLB-1785903337-antena-hd-para-tv-samsung-lg-smart-led-lcd-4k-3d-intext-uhf-_JM#reco_item_pos=1&reco_backend=machinalis-homes-pdp-boos&reco_backend_type=function&reco_client=home_navigation-trend-recommendations&reco_id=aeb505a5-a840-4908-b3da-189a1485b6c4&c_id=/home/navigation-trends-recommendations/element&c_element_order=2&c_uid=a021874e-5b64-4909-a39e-9d3bcf637e1b'
        ,'https://produto.mercadolivre.com.br/MLB-1931872137-smartphone-samsung-galaxy-m32-tela-64-128-gb-6-gb-ram-preto-_JM?variation=89413786390#reco_item_pos=1&reco_backend=promotions-sorted-by-score-mlb-A&reco_backend_type=low_level&reco_client=home_seller-promotions-recommendations&reco_id=c630912d-ee4e-4ced-b9ed-e672596a8d68&c_id=/home/promotions-recommendations/element&c_element_order=2&c_uid=cc906c30-0caf-4779-ac91-b70192ca1bca'
    ]
    for url in urls:
        scrapping_mercado_livre(url)